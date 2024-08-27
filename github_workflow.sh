sudo rm -rf data_service/migrations
docker compose -f github-workflow.yml --exit-code-from data_service up