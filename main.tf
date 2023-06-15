module "gce-container" {
  source = "terraform-google-modules/container-vm/google"
  version = "~> 2.0" 

  container = {
    image = "ubuntu-os-cloud/ubuntu-2004-lts"
  }
}

### BEGIN BOOT NODE ###

resource "google_storage_bucket_object" "fennel-protocol-boot-startup" {
  name   = "fennel-protocol-boot-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-boot-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-boot-ip" {
  name = "fennel-protocol-boot-ip"
}

resource "google_compute_instance" "fennel-protocol-boot" {
  name         = "fennel-protocol-boot-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-boot-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-boot-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-boot-ip-address" {
  name   = "fennel-protocol-boot-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-boot-ip.address
}

### END BOOT NODE ###

### BEGIN VALIDATOR NODE ###

resource "google_storage_bucket_object" "fennel-protocol-validator-startup" {
  name   = "fennel-protocol-validator-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-validator-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-validator-ip" {
  name = "fennel-protocol-validator-ip"
}

resource "google_compute_instance" "fennel-protocol-validator" {
  name         = "fennel-protocol-validator-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-validator-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-validator-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-validator-ip-address" {
  name   = "fennel-protocol-validator-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-validator-ip.address
}

### END VALIDATOR NODE ###

### BEGIN VALIDATOR 2 NODE ###

resource "google_storage_bucket_object" "fennel-protocol-validator-2-startup" {
  name   = "fennel-protocol-validator-2-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-validator-2-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-validator-2-ip" {
  name = "fennel-protocol-validator-2-ip"
}

resource "google_compute_instance" "fennel-protocol-validator-2" {
  name         = "fennel-protocol-validator-2-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-validator-2-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-validator-2-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-validator-2-ip-address" {
  name   = "fennel-protocol-validator-2-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-validator-2-ip.address
}

### END VALIDATOR 2 NODE ###

### BEGIN COLLATOR 1 NODE ###

resource "google_storage_bucket_object" "fennel-protocol-collator-1-startup" {
  name   = "fennel-protocol-collator-1-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-collator-1-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-collator-1-ip" {
  name = "fennel-protocol-collator-1-ip"
}

resource "google_compute_instance" "fennel-protocol-collator-1" {
  name         = "fennel-protocol-collator-1-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-collator-1-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-collator-1-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-collator-1-ip-address" {
  name   = "fennel-protocol-collator-1-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-collator-1-ip.address
}

### END COLLATOR 1 NODE ###

### BEGIN COLLATOR 2 NODE ###

resource "google_storage_bucket_object" "fennel-protocol-collator-2-startup" {
  name   = "fennel-protocol-collator-2-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-collator-2-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-collator-2-ip" {
  name = "fennel-protocol-collator-2-ip"
}

resource "google_compute_instance" "fennel-protocol-collator-2" {
  name         = "fennel-protocol-collator-2-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-collator-2-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-collator-2-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-collator-2-ip-address" {
  name   = "fennel-protocol-collator-2-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-collator-2-ip.address
}

### END COLLATOR 2 NODE ###

### BEGIN COLLATOR R NODE ###

resource "google_storage_bucket_object" "fennel-protocol-collator-rococo-startup" {
  name   = "fennel-protocol-collator-rococo-terraform-start.sh"
  bucket = "whiteflag-0-admin"
  source = "fennel-protocol-collator-rococo-terraform-start.sh"
  content_type = "text/plain"
}

resource "google_compute_address" "fennel-protocol-collator-rococo-ip" {
  name = "fennel-protocol-collator-rococo-ip"
}

resource "google_compute_instance" "fennel-protocol-collator-rococo" {
  name         = "fennel-protocol-collator-rococo-instance"
  machine_type = "e2-standard-2"
  zone         = "us-east1-b"

  can_ip_forward = true
  tags = ["public-server"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-11"
      size = "50"
    }
  }

  network_interface {
    network    = "whiteflag-sandbox-vpc"
    subnetwork = "public-subnet"
     access_config {
      nat_ip = google_compute_address.fennel-protocol-collator-rococo-ip.address
    }
  }

 metadata = {
    startup-script-url = "gs://whiteflag-0-admin/fennel-protocol-collator-rococo-terraform-start.sh"
    gce-container-declaration = module.gce-container.metadata_value
    google-logging-enabled    = "true"
    google-monitoring-enabled = "true"
  }
 
  service_account {
    scopes = ["cloud-platform"]
  }
}

resource "google_storage_bucket_object" "fennel-protocol-collator-rococo-ip-address" {
  name   = "fennel-protocol-collator-rococo-ip.sh"
  bucket = "whiteflag-0-admin"
  content = google_compute_address.fennel-protocol-collator-rococo-ip.address
}

### END COLLATOR 2 NODE ###