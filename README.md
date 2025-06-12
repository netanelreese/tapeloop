# TapeLoop
Rust port of Rockbox!

## Introduction

I've drank the Rust kool-aid, I love Rockbox. What else do you need to know?

## Objectives

#### Source Code Translation

- [ ] Bootloader for iPod

- [ ] Firmware source files for iPod

- [ ] Library Files

- [ ] Tools files

- [ ] Utils files

#### Custom Work

## Contribution Guidelines

- All source files must be thoroughly documented.
- New functionality must have unit tests written before PR submission.
- All convention and code styling will follow rust-lang's standard.
- All binaries and libraries should have or have very similar functionality to Rockbox's features, for sake of this being a plug-and-play application.
- Contributions must be signed.
- Versioning will follow a `<major>.<minor>.<bugfix>` standard. 
- All major and minor changes should be committed to a branch off of the latest release candidate (`rc/X.X.0`) branch. Once enough features have been added to warrant a release, the release candidate branch will be merged into the main branch.
  - Major versions are defined as breaking changes.
  - Minor versions are defined as non-breaking changes.
- All bugfixes should be committed to a bugfix branch (`bug/description-of-bug`). The bugfix branch may be merged into main, once it is the bugfix digit of the version number should be incremented.
- Pull Requests must have a description of all changes and how it was tested.

## FAQ
