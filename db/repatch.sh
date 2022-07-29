#!/bin/bash

mv src/schema.patch src/schema.patch.old
touch src/schema.patch
mv src/schema.rs src/schema.rs.cmp
diesel migration run
diff src/schema.rs src/schema.rs.cmp > src/schema.patch
rm src/schema.rs.cmp
rm src/schema.patch.old
diesel migration run

