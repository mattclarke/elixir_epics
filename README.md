# Elixir EPICS forwarder

A recreation of the EPICS Forwarder but in Elixir.

The motivation is to get more experience using Elixir and OTP by recreating an application I am familiar with.

Interesting things it does:
 - Uses GenServer(s)
 - Uses a Port to call an external program, in this case pvmonitor
 - Has a dynamic supervisor
 - Talks to Apache Kafka
 - Uses Rustler to call Rust code

## Running
First time only:
```
> mix deps.get
> chmod +x run_wrapper
```
Running:
```
> iex -S mix
```

### Adding monitors by hand
From within iex:
```
> ElixirEpics.MonitorSupervisor.start_child("SIMPLE:VALUE2")
```

## How it works
### Ports
- The program creates a GenServer that uses the [Port module](https://hexdocs.pm/elixir/Port.html) to create a pvmonitor process to monitor a port.
- The call is via the `run_wrapper` script as that makes sure that the pvmonitor process is cleaned up if the GenServer is stopped. See [zombie operating system processes](https://hexdocs.pm/elixir/Port.html#module-zombie-operating-system-processes) for more information.
- The port passes data to the GenServer via stdout. 
- pvmonitor is configured to format the PV data as JSON, so it is simple to recreate it in the GenServer.

### Apache Kafka
- Mostly taken care of by Brod.

## Possible improvements
- Store the flatbuffer in the state, so we don't need to create it on when resending it periodically.
- The topic is hard-coded.
- Use ETS just for experience of using it.
- Could the need for the `run_wrapper` be removed by registering the pid of the port and killing it in the supervisor when required. I guess it would be orphaned if BEAM crashed though, so perhaps not a great idea...
