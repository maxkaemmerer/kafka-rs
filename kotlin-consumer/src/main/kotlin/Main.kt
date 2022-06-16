import org.apache.kafka.clients.consumer.KafkaConsumer
import org.apache.kafka.clients.consumer.Consumer
import org.apache.kafka.common.serialization.StringDeserializer
import java.time.Duration
import java.util.Properties;

private fun createConsumer(): Consumer<String, String> {
    val props = Properties()
    props["bootstrap.servers"] = "kafka:9092"
    props["group.id"] = "kotlin-test-group"
    props["key.deserializer"] = StringDeserializer::class.java
    props["value.deserializer"] = StringDeserializer::class.java
    return KafkaConsumer(props)
}

fun main() {
    Thread.sleep(15_000)
    val consumer = createConsumer()
    consumer.subscribe(listOf("test-topic"))

    while (true) {
        val records = consumer.poll(Duration.ofSeconds(1))
        records.iterator().forEach {
            val message = it.value()
            println(message)
        }
    }
}