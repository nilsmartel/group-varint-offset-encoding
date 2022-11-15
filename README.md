# Group Varint Offset Encoding

- Compressed integers in blocks of 3.
- Has good compretion rate, even with outliers.
- Utilized offset encoding to store less bytes.
- Appends zeros to your data, if it can't be grouped into 3. Manually keep track of the exact amount if needed.
