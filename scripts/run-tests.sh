#!/bin/bash
dfx build backend

POCKET_IC_BIN="../../bin/pocket-ic" cargo test