#!/bin/bash

docker run -it --rm --name rust-debugger -v ${PWD}:/app rust /bin/bash
