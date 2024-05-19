# Anki GPT Integration - Desktop Edition

## Introduction

This project is a rewrite of my previous project last year where I wrote an integration for Anki to provide an interface to allow for generating flashcards with a Chat GPT model from the Open AI API and extracting text from a PDF file. In this version of the project, I rewrote it with Rust to run a desktop app. This avoids the need to run a backend and frontend, or bundled Docker container I used to run the previous version. 

### Technologies

This rewritten project still relies on TypeScript and React for the frontend which is run within Tauri. The following technologies are used: 
- Rust
- Tauri
- TypeScript
- React

### Todo
- [] Github workflow for releases to build the application and offer downloadable binaries 