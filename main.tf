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
  machine_type = "e2-small"
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

### END BOOT NODE ###

### BEGIN VALIDATOR NODE ###
#1 Polkadot boot node, one secondary validator, then two Fennel collators

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
  machine_type = "e2-small"
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
