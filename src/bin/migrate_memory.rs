//! Migration tool to convert memory_stream.json to SQLite two-tier system

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Import the types we need (copying minimal structures to avoid module dependencies)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OldMemory {
    id: String,
    content: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    entities: Vec<String>,
    connections: Vec<String>,
    memory_type: String,
    emotional_valence: f32,
}

#[derive(Debug, Deserialize)]
struct OldMemoryStream {
    memories: Vec<OldMemory>,
}

fn main() -> Result<()> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  VI3 Memory Migration Tool                            ║");
    println!("║  Converting JSON → SQLite Two-Tier System             ║");
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    // Read old memory_stream.json
    let old_path = "data/memory_stream.json";
    
    if !Path::new(old_path).exists() {
        println!("✓ No old memory_stream.json found - starting fresh!");
        return Ok(());
    }

    println!("📖 Reading old memory file: {}", old_path);
    let contents = std::fs::read_to_string(old_path)?;
    let old_stream: OldMemoryStream = serde_json::from_str(&contents)?;
    
    println!("   Found {} memories to migrate", old_stream.memories.len());
    println!();

    // Initialize new SQLite system
    println!("🔧 Initializing SQLite databases...");
    
    use rusqlite::{params, Connection};
    
    // Create active memory database
    let active_db = Connection::open("data/active_memory.db")?;
    init_active_schema(&active_db)?;
    
    // Create archive index
    let archive_index = Connection::open("data/archive_index.db")?;
    init_archive_schema(&archive_index)?;
    
    println!("   ✓ Databases created");
    println!();

    // Migrate memories
    println!("💾 Migrating memories...");
    
    // Decide which memories go to active vs archive
    // Keep most recent 200 in active, rest go to archive
    let active_limit = 200;
    let mut memories = old_stream.memories;
    memories.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    let (active_memories, archive_memories) = if memories.len() > active_limit {
        let split_point = active_limit;
        (memories[..split_point].to_vec(), memories[split_point..].to_vec())
    } else {
        (memories, Vec::new())
    };

    // Insert active memories
    for memory in &active_memories {
        let entities_json = serde_json::to_string(&memory.entities)?;
        let connections_json = serde_json::to_string(&memory.connections)?;

        active_db.execute(
            "INSERT INTO memories (id, content, timestamp, memory_type, emotional_valence, entities, connections)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                memory.id,
                memory.content,
                memory.timestamp.timestamp(),
                memory.memory_type,
                memory.emotional_valence,
                entities_json,
                connections_json,
            ],
        )?;

        // Add to entity index
        for entity in &memory.entities {
            active_db.execute(
                "INSERT OR IGNORE INTO entity_index (entity, memory_id) VALUES (?1, ?2)",
                params![entity, memory.id],
            )?;
        }
    }
    
    println!("   ✓ Migrated {} memories to active database", active_memories.len());

    // Archive old memories if any
    if !archive_memories.is_empty() {
        std::fs::create_dir_all("data/memory_archive")?;
        
        use std::collections::HashMap;
        
        // Group by month
        let mut by_month: HashMap<String, Vec<OldMemory>> = HashMap::new();
        for memory in &archive_memories {
            let month_key = memory.timestamp.format("%Y-%m").to_string();
            by_month.entry(month_key).or_insert_with(Vec::new).push(memory.clone());
        }
        
        for (month, month_memories) in by_month {
            let month_dir = format!("data/memory_archive/{}", month);
            std::fs::create_dir_all(&month_dir)?;
            
            let archive_file = format!("{}/migrated_archive.json", month_dir);
            let relative_path = format!("{}/migrated_archive.json", month);
            
            let json = serde_json::to_string_pretty(&month_memories)?;
            std::fs::write(&archive_file, json)?;
            
            // Add to archive index
            for memory in &month_memories {
                let entities_json = serde_json::to_string(&memory.entities)?;
                let connections_json = serde_json::to_string(&memory.connections)?;
                let content_preview = memory.content.chars().take(200).collect::<String>();

                archive_index.execute(
                    "INSERT OR REPLACE INTO archive_metadata 
                     (id, file_path, timestamp, entities, emotional_valence, memory_type, content_preview, connections)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        memory.id,
                        relative_path,
                        memory.timestamp.timestamp(),
                        entities_json,
                        memory.emotional_valence,
                        memory.memory_type,
                        content_preview,
                        connections_json,
                    ],
                )?;
            }
        }
        
        println!("   ✓ Archived {} older memories to JSON", archive_memories.len());
    }

    println!();
    println!("🎉 Migration complete!");
    println!();
    println!("📊 Summary:");
    println!("   • Active memories: {}", active_memories.len());
    println!("   • Archived memories: {}", archive_memories.len());
    println!("   • Total migrated: {}", active_memories.len() + archive_memories.len());
    println!();
    println!("💡 Next steps:");
    println!("   1. Your old memory_stream.json is preserved (backup)");
    println!("   2. New system uses: data/active_memory.db + data/memory_archive/");
    println!("   3. You can now run VI3 normally!");
    println!();

    Ok(())
}

fn init_active_schema(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            memory_type TEXT NOT NULL,
            emotional_valence REAL NOT NULL,
            entities TEXT NOT NULL,
            connections TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entity_index (
            entity TEXT NOT NULL,
            memory_id TEXT NOT NULL,
            PRIMARY KEY (entity, memory_id),
            FOREIGN KEY (memory_id) REFERENCES memories(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memories_timestamp ON memories(timestamp DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_entity_index_entity ON entity_index(entity)",
        [],
    )?;

    Ok(())
}

fn init_archive_schema(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS archive_metadata (
            id TEXT PRIMARY KEY,
            file_path TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            entities TEXT NOT NULL,
            emotional_valence REAL,
            memory_type TEXT,
            content_preview TEXT,
            connections TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_archive_timestamp ON archive_metadata(timestamp DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_archive_entities ON archive_metadata(entities)",
        [],
    )?;

    Ok(())
}

