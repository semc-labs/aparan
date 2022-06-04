[![Actor-based, monitored, composable robotics programming](/docs/assets/banner.svg)](https://github.com/semc-labs/aparan)

<h1 align="center">Aparan</h1>

<h4 align="center">
    <a href="https://github.com/orgs/semc-labs/projects/2/views/1?filterQuery=aparan">Roadmap</a>
  | <a href="https://discord.gg/uxGFjp65pK">Community</a>
</h1>

<p align="center"> Actor-based, monitored, strongly typed robotics programming </p>

**Aparan** is an actor-based, monitorable, strongly typed library for creating robotic programs. By combining scheduling, multi-threaded messaging, and a powerful actor model, one can swiftly program robots in-situ. For more information about features, read [our documentation](https://semc-labs.github.io/aparan).

## Features

 * **Strongly typed**. Catch errors with your robot before it's even built.
 * **Composable**. Glue modules of code together with `Plugins`.
 * **Actor-based**. Leveraging [`actix`](https://lib.rs/crates/actix), send messages in a clear, efficient manner.
 * **Monitored**. Monitoring with [`tracing`](https://lib.rs/crates/tracing) is as easy as 1-2-3.
 * **Hot-pluggable**. Enable, disable, or reload plugins at any time.
 * **Schedulable**. Use [`clokwerk`](https://lib.rs/crates/clokwerk) to schedule recurring tasks.
 * **Cross-platform**. With a large amount of supported targets, Aparan should work anywhere, out-of-the-box.
 * **`no-std`**. With some limitations, disable the standard library and run Aparan raw.

## Getting started

Ready? Just add `aparan` to your crate's dependencies, open the documentation, and go!

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Furthermore, please make sure to update tests as appropriate.
