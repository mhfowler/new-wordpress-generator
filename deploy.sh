#!/usr/bin/env bash
# exit when any command fails
set -e

# deploy
rsync -avzh --exclude target --exclude .idea --exclude .git -e "ssh -i /Users/maxfowler/.ssh/do_rsa" . root@67.207.73.230:/srv/wordpress-generator/

echo "++ generating new wordpress folder on droplet"
ssh -i /Users/maxfowler/.ssh/do_rsa root@67.207.73.230 'cd /srv/wordpress-generator; echo "++ running on droplet"; /root/.cargo/bin/cargo run -- -n kpv2 -p 8007 -d kpv2.canalswans.net'


