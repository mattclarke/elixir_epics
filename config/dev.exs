import Config

config :brod,
  clients: [
    kafka_client: [
      endpoints: [localhost: 9092],
      auto_start_producers: true
    ]
  ]

config :pv_monitor,
  path: "/opt/epics/base/bin/darwin-aarch64/pvmonitor"
