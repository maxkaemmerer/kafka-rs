import { Kafka } from "kafkajs"

(async () => {
  await (new Promise((res) => {setTimeout(res, 15000)}));

  const kafka = new Kafka({
    clientId: 'node-consumer',
    brokers: ['kafka:9092'],
  });

  const consumer = kafka.consumer({ groupId: 'node-test-group' })

  await consumer.connect()
  await consumer.subscribe({ topic: 'test-topic', fromBeginning: true })

  await consumer.run({
    eachMessage: async ({ topic, partition, message }) => {
      console.log(message.value?.toString());
    },
  })
})()