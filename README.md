# Elixir and EPICS

An example of using a GenServer with a Port to call an external program, in this case pvmonitor.

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

## How it works
- The program creates a GenServer that uses the [Port module](https://hexdocs.pm/elixir/Port.html) to create a pvmonitor process to monitor a port.
- The call is via the `run_wrapper` script as that makes sure that the pvmonitor process is cleaned up if the GenServer is stopped. See [zombie operating system processes](https://hexdocs.pm/elixir/Port.html#module-zombie-operating-system-processes) for more information.
- The port passes data to the GenServer via stdout. 
- pvmonitor is configured to format the PV data as JSON, so it is simple to recreate it in the GenServer.

## Possible improvements
- The PV is currently hard-coded.
- Be able to monitor multiple PVs using multiple BEAM processes.
- Could the need for the `run_wrapper` be removed by registering the pid of the port and killing it in the supervisor when required. I guess it would be orphaned if BEAM crashed though, so perhaps not a great idea...
