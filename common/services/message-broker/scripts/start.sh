rabbitmq-server -detached

echo "⏳ Waiting for RabbitMQ to start..."
max_attempts=30
attempt=0

while [ $attempt -lt $max_attempts ]; do
  if rabbitmqctl status >/dev/null 2>&1; then
    echo "✅ RabbitMQ is ready!"
    break
  fi
  
  echo "   Attempt $((attempt + 1))/$max_attempts - waiting 2s..."
  sleep 2
  attempt=$((attempt + 1))
done

if [ $attempt -eq $max_attempts ]; then
  echo "❌ RabbitMQ failed to start within 60 seconds"
  exit 1
fi

echo "🔧 Enabling RabbitMQ management plugin..."
rabbitmq-plugins enable rabbitmq_management