<p align="center">
  <a href="https://put.com">
    <img alt="PUT" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# PUT Geyser Plugin Interface

This crate enables an plugin to be added into the PUT Validator runtime to
take actions at the time of account updates or block and transaction processing;
for example, saving the account state to an external database. The plugin must
implement the `GeyserPlugin` trait. Please see the detail of the
`geyser_plugin_interface.rs` for the interface definition.

The plugin should produce a `cdylib` dynamic library, which must expose a `C`
function `_create_plugin()` that instantiates the implementation of the
interface.

The https://github.com/put-labs/put-accountsdb-plugin-postgres repository
provides an example of how to create a plugin which saves the accounts data into
an external PostgreSQL databases.

More information about PUT is available in the [PUT documentation](https://docs.put.com/).


