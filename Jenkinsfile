pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        echo 'Starting build'
        sh 'cargo build '
      }
    }
    stage('Done') {
      steps {
        echo 'Success'
      }
    }
  }
}