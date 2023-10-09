# Nebula

### Serverless Function-as-a-Service (FaaS) Platform Built in Rust

<!-- ![Nebula Banner](/path-to-your-banner-if-you-have-one.png) -->

## About

Nebula is an experimental Function-as-a-Service (FaaS) platform for my Master Thesis, offering a simplified user experience for deploying and executing code functions, aimed to investigate and analyze the performance and behavior of running functions as WebAssembly modules compared to containerized execution using Docker.

### Key Concepts

- **Efficiency and Performant Execution**: Nebula has been designed to examine the efficiency and startup times between Docker-based execution and WebAssembly modules. This exploration provides insights into the behaviors and performance characteristics of both approaches, offering an experimental environment to explore the practicality of WebAssembly in a serverless context.

- **Docker & WebAssembly Side by Side**: Nebula is not bound to a single execution environment. Instead, it allows the execution of functions within Docker containers as well as WebAssembly modules, providing flexibility and a range of deployment options to best suit different function requirements.

- **Ease of Use**: Designed with a focus on usability, Nebula enables developers to seamlessly deploy and manage their functions, without being concerned about the underlying infrastructure. It abstracts away the complexities related to infrastructure management, allowing developers to focus solely on writing the function logic.

### Architecture

- **Axum**: Leveraging the Axum framework, Nebula provides a high-level and type-safe web framework built on Tokio, focusing on ensuring correctness and safety without sacrificing performance.

- **WebAssembly**: WebAssembly functions allow for lightweight and fast-execution capabilities, making them a focal point for the investigation of sub-millisecond startup times and resource-efficient execution.

- **Docker**: Nebula is capable of executing Docker-based functions, facilitating broader compatibility and execution capabilities. This allows for an intriguing comparative study against WebAssembly in terms of startup times and resource utilization.

- **Metrics and Observability**: With the inherent capability of metric collection, Nebula provides insights into various operational metrics, such as startup time and execution duration, allowing users to scrutinize and comprehend the performance dynamics of their deployed functions.

## Motivation

Nebula was initiated as a part of a Master’s thesis, intending to delve into the exploration of serverless computing landscapes, with a specific emphasis on understanding the nuances between containerized function execution and WebAssembly-based function execution. This platform acts as a substrate for research and experimentation, enabling insights into the practical aspects and theoretical underpinnings of serverless paradigms.

## License

[MIT License](LICENSE.md)
