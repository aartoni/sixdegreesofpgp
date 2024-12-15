# Six Degrees of PGP

Work in progress.

## How it works

1. cache a [dump of the Cyberbits keyserver](https://mirror.cyberbits.eu/hockeypuck/dump/) via [rsync](https://github.com/rsyncproject/rsync);
2. parse the dump using [Sequoia-PGP](https://gitlab.com/sequoia-pgp/sequoia/-/tree/main/);
3. load public keys as nodes and signatures as edges in an [embedded neo4j database](https://neo4j.com/blog/neo4j-as-an-embedded-database-when-embedding-graph-db-make-sense/);
4. query via [neo4rs](https://github.com/neo4j-labs/neo4rs);

## Inspiration

- [this message](https://matrix.to/#/!aIgZXmFayJJhxsRtgN:matrix.org/$sdUOTGnUt52J109HXcPzwULhac09A3TDY8z2A55qLt4?via=matrix.org) on the [Matrix room of the Church of Cryptography](https://matrix.to/#/#churchofcrypto:matrix.org);
- [Six Degrees of Wikipedia](https://github.com/jwngr/sdow).

## Extra resources

- [Sequoia Web of Trust](https://gitlab.com/sequoia-pgp/sequoia-wot): a Rust library and tool for authenticating bindings between User IDs and certificates using OpenPGP's web of trust.
