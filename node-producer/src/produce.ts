import { Kafka } from "kafkajs"



(async () => {
  await (new Promise((res) => {setTimeout(res, 15000)}));

  const kafka = new Kafka({
    clientId: 'node-gateway',
    brokers: ['kafka:9092'],
  });
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
        { value: Buffer.from('Hello Kafka Consumers!') },
      ],
    });
  }

  await producer.disconnect()
})()