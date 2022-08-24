[![CodeFactor](https://www.codefactor.io/repository/github/hegde-atri/fileorganizer/badge)](https://www.codefactor.io/repository/github/hegde-atri/fileorganizer)

# ! I plan to re-do this project in rust in the near future !

# File Organizer

I made this project that organizes files in a folder based on specified options.
This was made to organize recovered data from my hard drive. Suitable for photos, videos and word documents.

# How to use
You can download the application in the [releases page.](https://github.com/hegde-atri/FileOrganizer/releases)

- You select the source and target paths by clicking on the select folder button below their label.
- If you want to sort all files in the source path, leave file type empty. If you want to only sort 1 type of file, then type its extension in the file type box. Example: ".jpg"
- You can organize by the month and year, or only the year of the file taken from the last modified metadata property. If None is selected, the selected file type will just be moved to Target path.
- Scan subfolder checks for files inside of folders in the source path.
- Auto detect subfolders creates folders at the target path for every extension it finds.

Then press Organize, you should be able to see the progress in the progress bar and its percentage right above it.

## Disclaimer
- There is not enough validation checks so make sure to test it with small sample to see if it works as intended.
- If moving fails, then your files should be safe. Some will be in the target path, and the rest in the source path (the failed  to move files.)
If this happens, please send me the Exception.txt file created in the target path.

