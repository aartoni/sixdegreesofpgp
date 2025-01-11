# Six Degrees of Frontend

A standalone frontend for "Six Degrees of" projects, forked from [Six Degrees of Wikipedia](https://www.sixdegreesofwikipedia.com/), refer to its [README.md](https://github.com/jwngr/sdow/blob/master/README.md) for more details on that project.

## Usage

The easiest way to use this repo is having it as a subtree of your project.

```sh
cd your-project
git subtree add --prefix=frontend https://github.com/aartoni/sixdegreesoffrontend.git main --squash
```

Then, adding it to your `compose.yml`.

```yml
services:
  api: # ...
  db:  # ...
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
```

Finally, changing the variables in `.env` and build the Docker image. The easiest way to add custom features would be forking this repository, then repeating the above steps.

## Changes

- environment variables support;
    - automatic substitution in the [web app manifest](https://developer.mozilla.org/en-US/docs/Web/Manifest) and HTML files;
- Google Analytics removed;

## Contributing

Contributions to the project are welcome! See the [contribution page](./.github/CONTRIBUTING.md) for
details on how to get everything set up in your local environment.
