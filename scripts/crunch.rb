require "progress"

input_path = ARGV[0] || "btsf/fred.btsf"
output_path = ARGV[1] || "btsf/fred-small.btsf"
input = File.open(input_path,"r")
output = File.open(output_path,"w")

# Read Header
version, file_header_size, rec_header_size, num_records = input.read(4*4).unpack("L*")
input.seek(file_header_size - 4*4, :CUR)

# Write Header
output.print [version, file_header_size, rec_header_size, num_records].pack("L*")

num_records.times.with_progress do
  # Read record header
  num_points, name_length = input.read(2*4).unpack('L*'.freeze)
  input.seek(rec_header_size - 2*4, :CUR)

  # Read record data
  name = input.read(name_length)
  points = num_points.times.map do
    input.read(2*4).unpack("le")
  end

  # Filter points
  last_time = nil
  points.reject! do |d,v|
    time = Time.at(d)
    toss = last_time && (time.month == last_time.month && time.year == last_time.year)
    last_time = time unless toss
    toss
  end

  # Write record data
  output.print [points.length, name_length].pack("L*")
  output.print name
  points.each do |d,v|
    output.print [d, v].pack("le")
  end
end

# Data
# data.each do |name, points|
# end
