sudo rm -rf data_service/migrations
docker compose -f --exit-code-from github-workflow.yml up