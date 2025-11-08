# Project H01 Authoritative Server

## Overview

This is the Authoritative server for the Chikin Buildak's Project, Project H01. Currently still in development.

## NOTE

Do not blindly follow the practice used in this project, as the developer working on this still learning the framework and the Rust programming language

## What this system does

This system manage runtime for each ECS world for each room created inside the game system.
The communication between this server and [client](https://github.com/ChikinBuldak/project-h01-client) is done through WebRTC protocol, by an intermediate [signaling server](https://github.com/ChikinBuldak/project-h01-signaling). Currently this system does not have a good security. This will be implemented later after the game is ready for production. Also this projects in the first place was a proof of concept
did by the programmer to learn Rust by a real project