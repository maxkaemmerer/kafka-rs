import { Kafka } from "kafkajs"

const kafka = new Kafka({
  clientId: 'node-gateway',
  brokers: ['kafka:9092'],
});

(async () => {
  const producer = kafka.producer({
    retry: {
      retries: 15
    }
  })

  await producer.connect()


  while(true){
    await (new Promise((res) => {setTimeout(res, 5000)}));
    await producer.send({
      topic: 'test-topic',
      messages: [
        { value: 'Hello KafkaJS user!' },
      ],
    });
  }

  await producer.disconnect()
})()