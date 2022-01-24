# Project doorbell

Project Doorbell is an idea that I got to know the managed AI AWS services and keep going to write some code in Rust.
Smart doorbells include a camera, microphone and speaker to allow for a two-way conversation between the doorbell and its smartphone app.
This project is concentrated on the backend side of communication. Therefore, I will not consider the hardware and the mobile app.
You can find a series of articles [here](https://dfrasca.hashnode.dev/series/project-doorbell)

### How it works ###

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/readme/citofono.jpeg)

- I ring my doorbell.
- The doorbell takes a picture of me.
- Will open a WebSocket connection
- Send an event to EventBridge to create the presignd url
- Send it back to the doorbell
- Doorbell upload the photo to S3
- S3 PUT will generate an event to EventBridge and execute the Step Function.
- Step Function is comparing the face

If we have a match:

- Generate a 6 digit code
- Send the code to the mobile app
- Insert the code in the doorbell keypad and go inside.

If we have do not have a match:

- Send error back to the doorbell
- Let the doorbell ring
- Send a photo to the mobile app

### HOW TO DEPLOY ###

The project will deploy the following AWS services:

- AWS Lambda functions
- Amazon API Gateway
- AWS Step Functions
- Amazon DynamoDB

Part of the initial setup can be read it [here](https://dfrasca.hashnode.dev/project-doorbell-infrastructure-setup)

This project require RUST and for basic setup please refer to [part 1](https://dfrasca.hashnode.dev/hello-serverless-rust) and [part 2](https://dfrasca.hashnode.dev/rust-using-lambda-arm64-architecture) of [RUST series](https://dfrasca.hashnode.dev/series/how-to-serverless-rust)

Assuming that your computer is setup, you need to build

```
make build
```

Once it is all built

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/readme/build.png) 

We can deploy all the applications. I use 

> --profile test 

Inside my MakeFile, and you may remove it.

```
make deploy
```

If everything is all working, you should have in your AWS Account something like this:

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/readme/cf.png)

Inside the rust-doorbell-websocket stack, you can find the WebSocket URL. It should look like

wss://xxxxx.execute-api.eu-central-1.amazonaws.com/prod

### Testing ###

[Postman Now Supports WebSocket APIs](https://blog.postman.com/postman-supports-websocket-apis/), and we will leverage this tool to simulate the flow.

This project is concentrated on the backend side of communication. Therefore, I will not consider the hardware and the mobile app. This means that we will send the 6 digit code back to the WebSocket channel.
In reality, this will not happen because the code or the person's preview at the doorbell will be sent to the mobile app.
I took the liberty to use the WebSocket to take some action.

1. Upload inside the bucket a passport type photo of you, and call it source.jpeg. This is the source photo that we are using to compare the photo taken by the doorbell
2. Open Postman
3. Create a new WebSocket Request
4. Past your address wss://xxxxx.execute-api.eu-central-1.amazonaws.com/prod
5. Click Connect

You should see something like:

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/readme/postman1.png)

Copy the return of the presigned url, something like

https://s3.eu-central-1.amazonaws.com/rust-doorbell-s3-sourcebucket-xxxxxxx/guest/Mc8_Pc81liACHoQ%3D/guest.jpeg?x-id=PutObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAYKMZLNCDZDGUNLLS%2F20220124%2Feu-central-1%2Fs3%2Faws4_request&X-Amz-Date=20220124T132346Z&X-Amz-Expires=300&X-Amz-SignedHeaders=host&X-Amz-Signature=c95bb0966496d807bbac9c37a097f49dc6eff2b569b9a7ed17319a4fc6352fc6&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEE0aDGV1LWNlbnRyYWwtMSJGMEQCICPze%2BYK0U9xGQa86RAgvxy1dAvgCAp9JGdsmDpR5KkdAiBNpTJOTW%2F7SW2CGnGJy%2B%2F5lEeUyhAtCU%2BJtoCTcQko%2FirEAgh3EAMaDDU3MjA4OTEzMzE5MSIMizWj6%2B91YdoTI4eKKqECMr5XmmjaW9KIm0Vwc16rc%2Fidr9EJbiPRsmA5I9pJ5tZpKaMaTFog%2FpMztrftAkXkZorwNK9f1wvJdUxDXHrHrZ00vBt6y%2FGSGHpmW1uZMmJLc%2FL8wZqxcm49caAioXtElK2xE30cHBCf3lw2rQXCi9SnUK0P0D6E0cB4jYfOSOPHh%2F9onVPHiYkENz0KBCFRBVTR68dEpQ4gYLsMyQvirNJKDocliNAyhFKZJ7jxHk2EiMw8FzZG67Zwgtk3ceVd4Kux0sVFvVW3dAs9nv7Yq5kotmv1HacSxBT0Jqv2rxaj203Vkje6g03phJxpLiq%2FBzLUO4JIdA8uN8rwkHgnzSlrE7AkkqnmU5p%2F3wCvZkvlg%2FkBLnKpzZ5DENijbiMXCzDi0LqPBjqbAck8ZFvJNPl30TerlHhU3jIKuyV%2B66iNeXfjoNqcnh2OM1WC0Yv5KIdBrxD8GnQC4jcM0hfxG%2FGAfzdcQ0JEo6lWLm8apkyGLDrNk2tgB2iML5rxgkLTmRDtJMlFCxAYkG8P1snbUKSXC0BzTJi7vdBz7RCXrvQo51SjI5Js1vXVLR0vNAQTFoVPR0pNNWcLvOSVe3Dq7n76D1jr


And open a new tab in postman using the link you got (remember to upload a recent photo of you) and click Send.

Go back to the WebSocket tab, and you should see the code in return.

![picture](https://github.com/ymwjbxxq/rust_doorbell/blob/main/readme/postman3.png)


### Cleanup ###
```
make delete
```