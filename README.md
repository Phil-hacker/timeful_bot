# Timeful Bot

A program to create a timeful event and send the event to a discord webhook.

When started it will try to figure out when the next week starts and create a timeful event for that week.

The config file will be loaded either from
 1. The path that is supplied as the first argument
 2. `./config.toml` if no path is specified

The config file has the following format:

```toml
[[event]]
name = "My Event name"
start_hour = 16
# Optional, default is 0 minutes
start_minute = 15
duration = 8
# Optional, default is 15 minutes
time_increment = 5
# %link% gets replaced with the event link
message = "# New Timeful Link\n%link%"
webhooks = [
    "https://discord.com/api/webhooks/random stuff goes here",
]
```

You can specify multiple events in a single config file, multiple config files are currently not supported.

Currently doesn't retry to create the event if the timeful servers are not available.