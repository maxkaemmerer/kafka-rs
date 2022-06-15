import { Kafka } from "kafkajs"

const kafka = new Kafka({
  clientId: 'node-consumer',
  brokers: ['kafka:9092'],
});

(async () => {
  const consumer = kafka.consumer({ groupId: 'node-test-group' })

  await consumer.connect()
  await consumer.subscribe({ topic: 'test-topic', fromBeginning: true })

  await consumer.run({
    eachMessage: async ({ topic, partition, message }) => {
      console.log({
        value: message.value?.toString(),
      })
    },
  })
})()