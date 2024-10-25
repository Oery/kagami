# Kagami

**WIP: The crate should not be used yet, it is still in heavy development.**

Kagami is an implementation of the Minecraft protocol featuring a simple and easy to use API to modify and create packets through events.

## What can I do with this ?

Since it acts as a server between your client and the server, you can do pretty much anything the server can do, this includes :

- Server plugins on the client
- Visual Modifications / Spoofing
- Integrations with other apps
- Custom Commands
- Chat Based Events
- Scoreboard Scrapping

## Why using this over a mod loader ?

Mods are way more powerful than a proxy, but they require the game to run with a mod loader such as Fabric or Forge. Some closed source modded clients do not support loading more mods than what they offer. So when building a mod, you need to target a very specific userbase.

Since Kagami is a proxy, it doesn't need any access to the game, and could even run on the server, so it can be used with any client. This means it works the same on a vanilla client, or a closed source modded client. The only thing that will break is the game protocol depending on the game version.

As of now, Kagami is only compatible with Minecraft 1.8.9, but in the future it should support most versions of Minecraft.

## The API

I want to make the API as simple as possible, so that it is easy to use, even for Rust beginners. It contains a bunch of APIs to help you build using common features you probably need.

#### Callback API

Callbacks are function that will run every time the matching packet is received or sent.

One can simply be registered using a closure that takes the desired packet as argument, wrapped in a Context that will give access to more features, such as communicating with the client/server from inside a callback.

Here is an example of a basic command handler using directly the Chat packets. If the user types `/ping` in the chat, the proxy will respond with `Pong!`. Note that the command will not be sent to the server thanks to the `Actions::Filter` action.

```rust
proxy.register_callback(|ctx: &Context<client::Chat>| {
    if ctx.packet.message == "/ping" {
        ctx.source.send(Chat::new("Pong!"));
        return Actions::Filter;
    }

    Actions::Transfer // If no modification was made, the original data is sent
});
```

> [!NOTE]  
> Packets that have a callback assigned (by user or system) are the only ones that will be deserialized.

#### Command API

Commands are very useful to create interactions with the client. Some modded clients can even use macros to send text using only a key press. A user should be able to create simple commands without having to manually register chat events. Since most people will probably adds their own command system at some point, we might as well just include one.

```rust
// We start by defining an enum of all possible subcommands
enum FriendAction {
    Add,
    Remove,
    List,
}

// Then we describe the arguments of the commands
pub struct FriendCommand {
    pub action: FriendAction,
    pub name: Option<String>,
}

// Now we can register the command and implement the logic
proxy.commands.register('ping', |ctx: &Context<FriendCommand>| {
    match ctx.command.action {
        FriendAction::Add => add_friend(ctx.command.name.unwrap()),
        FriendAction::Remove => remove_friend(ctx.command.name.unwrap()),
        FriendAction::List => ctx.client.send(Chat::new("List of friends")),
    }
});
```

> [!NOTE]  
> It is safe to unwrap the name here, because it was validated by the command system.

Some nice features could be, more complex validation, more types like a Mojang player, holding both the username and UUID while requiring only the name to be fetched.

#### World API

In the future, I want to store the world state so its data can be accessed at any time. For example, if you want to access the player list, you would need to track the player list yourself, using callbacks on related packets. An easy API could make it much easier by letting you access the current state of the world :

```rust
proxy.commands.register('players', |ctx: &Context<PlayersCommand>|) {
    let players = ctx.world.players(); // Obtain players from World State
    let string = players.iter().map(|player| player.name).collect::<Vec<String>>().join(", ");
    ctx.client.send(Chat::new(string));
}
```

#### Window API

Commands are good, but sometimes what you need is an UI. The Window API will allow you to create windows that can be interacted with by the user.

## Actions

Since packets are directly editable, we need to tell the proxy what to do with them using Actions.

If you modify a packet, you should use `Actions::Modify`. It will ask the proxy to serialize the new packet and replace the original.

If you want to fully intercept the packet, without the destination knowing anything about it, you should use `Actions::Filter`.

If nothing happened to the packet, you should use `Actions::Transfer`. It will tell the proxy to forget about the packet and the origial bytes will be transmitted to the destination.

## Limitations

Since each app built using Kagami is one proxy, it is very unefficient to use multiple apps using Kagami at the same time. I want to try to find a solution to let multiple apps share the same proxy but it will probably take a lot of work.

Also, if you edit packets, you're directly changing the source of truth which means apps and mods who depends on what the game sees will break. For example, if you change usernames, bedwars overlays would break since they would get the wrong usernames. You can always add checks to make sure you don't break stuff you don't want to break but keep it in mind. The real solution would be to use Kagami for the overlay in the first place.

### Roadmap

- **Basic Protocol Support**

  - [x] State management
  - [x] Buffering large packets
  - [ ] Compression support
  - [ ] Hostname checks bypass

- **Features**

  - [x] Callbacks API
  - [ ] Context API
  - [ ] Command API
  - [ ] World API
  - [ ] Window API

- **Improvements**
  - [ ] Reverse Proxy mode
  - [ ] Multi-version support
  - [ ] Deep Performance optimizations
