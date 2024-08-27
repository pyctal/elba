sudo rm -rf data_service/migrations
docker compose -f github-workflow.yml up --exit-code-from data_service