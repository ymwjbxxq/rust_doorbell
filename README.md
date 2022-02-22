# Project doorbell

Project Doorbell is an idea that I got to know the managed AI AWS services and keep going to write some code in Rust.
Smart doorbells include a camera, microphone and speaker to allow for a two-way conversation between the doorbell and its smartphone app.
This project is concentrated on the backend side of communication. Therefore, I will not consider the hardware and the mobile app.
You can find a series of articles [here](https://dfrasca.hashnode.dev/series/project-doorbell)

## Structure ##

I have split this repository by microservices:

* The orginal idea is [here](https://github.com/ymwjbxxq/rust_doorbell/tree/main/services/v1)
* Session Manager service is [here](https://github.com/ymwjbxxq/rust_doorbell/tree/main/services/session-manager)

Even if they are not correlated, I keep the code separated because each part is a small experiment, and it is possible to see the evolution of libraries and techniques.

If we imagine this as a repository for a company, we will have different teams and accounts, and each of them will be responsible for a service.