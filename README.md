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
> ElixirEpics.MonitorSupervisor.start_child({"SIMPLE:VALUE2", "test_topic", "schema is ignored"})
> ElixirEpics.MonitorSupervisor.start_child({"SIMPLE:WAVE", "test_topic", "schema is ignored"})  
```

### Stopping a monitor by hand
From within iex:
```
# Assumes we know the pid
> p = pid(<the comma-separated pid>)
> GenServer.stop(p, :normal)
```
Anything other than `:normal` or `:shutdown` will automatically restart the process.

## How it works
### Ports
- The program creates a GenServer that uses the [Port module](https://hexdocs.pm/elixir/Port.html) to create a pvmonitor process to monitor a port.
- The call is via the `run_wrapper` script as that makes sure that the pvmonitor process is cleaned up if the GenServer is stopped. See [zombie operating system processes](https://hexdocs.pm/elixir/Port.html#module-zombie-operating-system-processes) for more information.
- The port passes data to the GenServer via stdout. 
- pvmonitor is configured to format the PV data in the `raw` format.
- Standard out has a limit to how much data it can output in one go, so for long waveforms the data may be split over multiple "updates". To handle this pvget is modified to print `=====\n` at the end:
```
pvget.cpp line 242 insert:

std::cout << "=====\n";
```

### Apache Kafka
- Mostly taken care of by Brod.

## Possible improvements
- Handle long[], double[], int[] (does that exist?)
- Cached values need to only be updated when value changes.
- Separate the monitor code into a separate testable module.
- How to remove monitors via code.
- The schema is hard-coded.
- Use ETS just for experience of using it.
