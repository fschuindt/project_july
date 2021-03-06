defmodule Streamer.MixProject do
  use Mix.Project

  def project do
    [
      app: :streamer,
      version: "0.1.0",
      build_path: "../../_build",
      config_path: "../../config/config.exs",
      deps_path: "../../deps",
      lockfile: "../../mix.lock",
      elixir: "~> 1.6",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:timex],
      mod: {Streamer.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:secure_random, "~> 0.5.1"},
      {:grpc, github: "tony612/grpc-elixir", branch: "master"},
      {:timex, "~> 3.1"}
    ]
  end
end
