from aruba import ping, send

def get_ping():

    # get the actual ping data
    ping_data = ping(3, "google.com")

    # here we do whatever we want with the actual data, string manipulation etc
    # very naive approach to structuring data
    send(ping_data)

get_ping()