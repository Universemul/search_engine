# search_engine

### Indexes Structure

The root directory can be changed using `path_data` in config.yaml
An index is defined by a directory. Inside, you have :
 * .sg files (or segments) which store your data relative to your index.
 * A `.meta` file where you will have the number of segments, the mapping of the index, the index's name
 * A `.lock` file only present when the index is lock


### Index

Only primary field can be use

### TODO
    * Display all index's name and size (route '/')
    * Display mapping of index (check .meta file) and size of index
    * Create Index
    * Push data into index (only create)
    * Get Data from index
    * Update data into index
