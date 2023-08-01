import Config

config :brod,
  clients: [
    kafka_client: [
      endpoints: [localhost: 9092],
      auto_start_producers: true
    ]
  ]
