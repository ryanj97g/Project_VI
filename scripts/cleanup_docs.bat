@echo off
REM Documentation Cleanup Script for VI3 V3.1
REM Consolidates 18 .md files down to 11 clean files

echo ========================================
echo VI3 Documentation Cleanup
echo ========================================
echo.

REM Step 1: Safety backup
echo [1/6] Creating backup...
if not exist docs_backup mkdir docs_backup
copy *.md docs_backup\ >nul 2>&1
echo    Backed up all .md files to docs_backup\
echo.

REM Step 2: Delete redundant files
echo [2/6] Deleting redundant files...
echo    - Deleting old README.md
del README.md
echo    - Deleting INDEX.md (merged into DOCUMENTATION_INDEX.md)
del INDEX.md
echo    - Deleting QUICKSTART.md (superseded by VI3_QUICKSTART.md)
del QUICKSTART.md
echo    - Deleting ALL_DONE.md (superseded by V3.1 docs)
del ALL_DONE.md
echo    - Deleting THANK_YOU.md (duplicate)
del THANK_YOU.md
echo.

REM Step 3: Rename files for clarity
echo [3/6] Renaming files...
echo    - README_VI3.md -^> README.md
ren README_VI3.md README.md
echo    - IMPLEMENTATION_COMPLETE.md -^> IMPLEMENTATION_COMPLETE_V3.0.md
ren IMPLEMENTATION_COMPLETE.md IMPLEMENTATION_COMPLETE_V3.0.md
echo.

REM Step 4: Count remaining files
echo [4/6] Verification...
set count=0
for %%f in (*.md) do set /a count+=1
echo    Found %count% .md files (should be 11)
echo.

REM Step 5: List final structure
echo [5/6] Final documentation structure:
echo.
echo Entry Points:
echo    - README.md
echo    - START_HERE.md
echo.
echo Navigation:
echo    - DOCUMENTATION_INDEX.md
echo.
echo User Guides:
echo    - BATCH_FILES_GUIDE.md
echo    - BATCH_FILES_ORDER.md  
echo    - VI3_QUICKSTART.md
echo.
echo V3.1 Documentation:
echo    - V3.1_RELEASE_NOTES.md
echo    - CHANGELOG_V3.1.md
echo    - VI_IDENTITY_INTEGRATION.md
echo    - COPY_LAST_2_GUIDE.md
echo    - INTEGRATION_COMPLETE_V3.1.md
echo.
echo Reference:
echo    - PROJECT_VI_V3_IMPLEMENTATION.md
echo    - IMPLEMENTATION_COMPLETE_V3.0.md
echo.

REM Step 6: Instructions
echo [6/6] Next steps:
echo.
echo    MANUAL UPDATES NEEDED:
echo    1. Update README.md (add V3.1 section)
echo    2. Update START_HERE.md (add V3.1 mention)
echo    3. Update DOCUMENTATION_INDEX.md (add V3.1 links)
echo.
echo    Then commit to git:
echo    git add .
echo    git commit -m "Documentation cleanup: 18-^>11 files"
echo    git push
echo.

echo ========================================
echo Documentation Cleanup Complete!
echo ========================================
echo.
echo Backup saved in: docs_backup\
echo.

pause

