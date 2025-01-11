# Contributing | SDOF

Thank you for contributing to Six Degrees of Frontend!

## Local setup

Contrary to [Six Degrees of Wikipedia's](https://github.com/jwngr/sdow), this repository only explains how to setup the frontend side for your "Six Degrees of" app.

### Initial setup

The first step is to clone the repo and move into the created directory.

```bash
git clone git@github.com:aartoni/sixdegreesoffrontend.git
cd sixdegreesoffrontend
```

The only required dependency is any [Node.js](https://nodejs.org) distribution (e.g, [`nvm`](https://github.com/nvm-sh/nvm)), after installing one, dependencies can be downloaded via `npm`.

```bash
cd website
npm i
```

### Recurring setup

To run this frontend, open a new tab and run the following commands from the repo root:

```bash
cd website
npm start
```

The service can be found at http://localhost:3000.

## Repo organization

Here are some highlights of the directory structure and notable source files

- [`.github`](.) - Contribution instructions as well as issue and pull request templates
- [`website`](../website) - The frontend React + TypeScript website, built using Vite
