docker exec -it src-kafka-1-1 /opt/kafka/bin/kafka-console-consumer.sh \
  --topic my-test-topic \
  --from-beginning \
  --bootstrap-server localhost:9092


docker exec -it src-kafka-1-1 /opt/kafka/bin/kafka-console-producer.sh \
  --topic my-test-topic \
  --bootstrap-server localhost:9092