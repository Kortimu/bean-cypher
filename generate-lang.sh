#!/bin/sh

egui-i18n-cli generate \
    --source-path ./src \
    --output-path ./assets/lang \
    --language en-GB \
    --language fi-FI \
    --language lv-LV \
    --ext ftl
