zmakin.gitbook.io/bns

Please read my Bitmap Naming System theory at the above address to understand they why and how of this code repository.

This repository is broken up into 3 sbranches. The main branch is how I started. It is the code i set up to run a bitmap indexer, prior to any implementation of the ordinals rust code into integration. This first open section would be just the base of the code and what i want it to accompplish. This would require running Bitcoin Core and ORD clients each up to date with their respective sections.

This second section labeled "bns overlay" is a standalone overlay of the bitmap indexer built using the ord rust code as a large denepndency of the code, but not as a direct addition to the ord client. This is most likely the first route to test and start indexing bitmap, and thus prepare for a public BNS blockheight to be set
