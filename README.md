# Anki GPT Integration - Desktop Edition

# Background

## Introduction

This project is a rewrite of my previous project last year where I wrote an integration for Anki to provide an interface to allow for generating flashcards with a Chat GPT model from the Open AI API and extracting text from a PDF file. In this version of the project, I rewrote it with Rust to run a desktop app. This avoids the need to run a backend and frontend, or bundled Docker container I used to run the previous version. 

### Version 1

I originally wrote the project using a Python backend web server and a React/TypeScript frontend. The program would then sync with Anki using a web socket (Socket.IO) from the Python web server (which runs locally on the users machine and syncs to Anki) to the frontend. This middle complexity of having a web socket and separate web server communicate to Anki proved to be a challenge in scalability. Additionally, I saw it as an opportunity to venture in the world of Rust and Tauri, leading to the rewritten project. Additionally, a desktop application seemed to align more with my vision for the project. Version 1 is visible [here](https://github.com/ryanarnouk/AnkiGPTIntegration)

I have copied over some details from the README of the version 1 project to this one. 

### Preface

As a Computer Science student, I do not often get the opportunity to use spaced repetition programs like Anki, compared to those in life sciences, for example. We often need to remember big ideas and interpretations rather than specific facts.

Therefore, combining machine learning with Anki to test 'big idea' questions while tapping into a powerful spaced repetition algorithm and software may provide a new learning experience for those of us not used to using Anki but still want to test and remember big picture ideas and concepts.

### Idea

I originally wanted to work on an application to generate question and answers from notes years ago after learning about the importance of testing yourself consistently during the learning process (namely active recall and spaced repetition). I put this project on the back-burner due to my inexperience with machine learning models. However, with the increased popularity of LLMs like ChatGPT, this project idea has become a lot more viable. 

And, thanks to the AnkiConnect add-on, rather than needing to create a spaced repetition/active recall algorithm on my own, the project can utilize Anki's proven power.

# Setup

### Technologies

This rewritten project still relies on TypeScript and React for the frontend which is run within Tauri. The following technologies are used: 
- Rust
- Tauri
- TypeScript
- React

### Setup

**Required technologies**

- Rust
- Cargo (Rust package manager)
- Node and NPM

Step 1: navigate to the `src` directory:
```
cd src
```

Step 2: install the frontend packages
```
npm install
```

Step 3: run the application with tauri in development mode
```
npm run tauri dev
```

**Building**

To build the application locally, you can use:
```
npm run tauri build
```


### Todo
- [ ] Github workflow for releases to build the application and offer downloadable binaries
- [ ] Refine bugs on initial load of application not matching with Anki until the card is refreshed
- [ ] Fix process with setting environment variable for Open AI API key. Storing the environment variable can be be persisted in the new desktop application rather than using environmnet variables that will refresh (requiring the user to repeatedly input their variable in each session).  
