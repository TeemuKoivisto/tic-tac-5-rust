# How to run

Install ansible eg `brew install ansible`

## Ubuntu server

1. Enter folder: `cd deploy`
2. Change hosts file: `cp hosts.example hosts`
3. Execute: `AWS_ACCESS_KEY_ID=xxx AWS_SECRET_ACCESS_KEY=xxx ansible-playbook -i hosts playbook.yml`
