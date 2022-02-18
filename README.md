# Project doorbell

Project Doorbell is an idea that I got to know the managed AI AWS services and keep going to write some code in Rust.
Smart doorbells include a camera, microphone and speaker to allow for a two-way conversation between the doorbell and its smartphone app.
This project is concentrated on the backend side of communication. Therefore, I will not consider the hardware and the mobile app.
You can find a series of articles [here](https://dfrasca.hashnode.dev/series/project-doorbell)

## Structure ##

Microservices split this repository:

* The orginal idea is [here](https://github.com/ymwjbxxq/rust_doorbell/tree/main/services/v1)
* Session Manager service is [here](https://github.com/ymwjbxxq/rust_doorbell/tree/main/services/session-manager)

Even if they are not correlated, I keep the code separated because each part is a small experiment.

The idea is to use some imagination and think that this is the repo of a big company, and different teams and accounts use each service.
