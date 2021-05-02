#!bin/sh

#githubとsshを接続するためのスクリプト

pubKeyName="id_rsa_github"

#ssh-agent起動
eval `ssh-agent`

#ssh-agentに公開鍵を登録しておく
ssh-add ~/.ssh/${pubKeyName}

#確認
ssh-add -l

#githubと繋がったか確認
ssh -T git@github.com
