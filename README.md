# Kagami

**WIP: The crate should not be used yet, it is still in heavy development.**

Kagami is a Minecraft proxy featuring a simple and easy to use API to modify and create packets through events.

## What can I do with this ?

Since it acts as a server between your client and the server, you can do pretty much anything the server can make you do, this includes :

- Visual Modifications
- Integrations with other apps
- Custom Commands
- Chat Based Events
- Scoreboard Scrapping

## Why using this over a mod loader ?

Mods are way more powerful than a proxy, but they require the game to run with a mod loader such as Fabric or Forge. Some closed source modded clients do not support loading more mods than what they offer. So when building a mod, you need to target a very specific userbase.

Kagami is a proxy, so it can be used with any client, and it does not require any mod loader. This means it works the same on a vanilla client, or a closed source modded client. The only thing that will break is the game version.

As of now, Kagami is only compatible with Minecraft 1.8.9, it might get support for newer versions someday but that is not guaranteed as it would require a lot of work (+ most closed source clients are way less popular).

## How to use

- Initialize a Proxy on a local port
- Connect to the proxy address to join the server

## Example

This is a simple example of a handler that interacts with chat messages sent by the client:

```rust
mc.handlers.add_write_handler(|packet: &mut client::Chat| {
    Box::pin(async move {
        // Edit the message if it contains "foo"
        if packet.message.contains("foo") {
            packet.message = "I never said that!".into();
            return PacketAction::Edit;
        }

        // Do not send the packet to the server if it contains "bar"
        else if packet.message.contains("bar") {
            return PacketAction::Filter;
        }

        // Send the packet to the server
        PacketAction::Default
    })
});
```

The read handlers are used to read packets from the server. They are triggered after a packet has been received and sent to the client. This means it is too late to modify the packet but it can still be read without adding any delay. If a packet is not modified, this should be used.

This is a simple example of a handler that reads the content of a packet:

```rust
mc.handlers.add_read_handler(|packet: &client::WindowClick| {
    Box::pin(async move { println!("Slot clicked: {:#?}", packet.item) })
});
```

## Actions

Actions are used to tell the proxy what to do with a packet.

Filter will remove the packet from the buffer, so it is not sent to the destination.
Edit will serialize the new version of the packet and replace the original one in the buffer.
Default will do nothing to the buffer.

## Limitations

Since each app built using Kagami is one proxy, it is very unefficient to use multiple apps using Kagami at the same time. I want to try to find a solution to let multiple apps share the same proxy but it will probably take a lot of work.

### Roadmap

- [ ] State management
- [x] Reading basic packets
- [x] Reading packets with NBT Data
- [x] Serializing basic packets
- [ ] Serializing packets with NBT Data
- [x] Events to read packets
- [x] Events to write packets
- [ ] Send packets to client from an event handler
- [ ] Send packets to server from an event handler
- [ ] Custom Commands Handler
- [ ] Custom Inventory GUIs
- [ ] Compressed packets support
- [ ] Support for server with hostname checks
- [ ] Auth support
