#!/bin/bash
set -e
opencv_interactive-calibration -t=charuco -sz=22.19 -w=7 -h=9 -pf=params.xml -v=calibration_video.mkv -d=0.2 --zoom=0.3 --save_frames

