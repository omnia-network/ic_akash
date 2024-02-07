#!/bin/bash

dfx deps deploy

dfx deploy backend --argument '(false)'
