# rs-media

[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
[![codecov](https://codecov.io/gh/reynn/rs-media/branch/master/graph/badge.svg)](https://codecov.io/gh/reynn/rs-media)
[![CI](https://github.com/reynn/rs-media/workflows/CI/badge.svg)](https://github.com/reynn/rs-media/actions?query=workflow%3ACI+branch%3Amaster)

## Project Overview

Media management and server combo written in Rust

## FAQ

### Why

Many current media management and delivery services are built on the grounds of
tech from decades ago that is difficult to maintain. Many of these same services
are finding it difficult to grow with many of our growing media libraries.

## Customizable

All pieces listed below will be installable via a plugin system, similar to build
systems like Jenkins, bamboo etc where there is a customizable system for all users.

Default setup would likely use a PostgreSQL database with IMDB, theTVDB, TheMovieDB
providers installed. Providers of pieces can be dependant on other providers in the
same category. Metadata discovery can depend on any other metadata subcategory but
not something from the database category?

## Pieces

Built using Rust and encouraging development of "plugin" crates in parallel.

### Database

### Meta-Data

#### Discovery

These are for discovering meta-data for all types of files from APIs.

Planned providers below are subject to change.

- [IMDB](https://www.imdb.org/)
- [theTVDB](https://www.thetvdb.org)
- [TheMovieDB](https://www.themoviedb.org)

#### Parsing

Getting existing meta-data into the library from discovery from the media server.

Planned providers below are subject to change.

- NFO
- JSON
- ZSH script
