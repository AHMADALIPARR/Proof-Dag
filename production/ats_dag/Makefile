PATSCC=patscc
PATSOPT=patsopt
ATSFLAGS=-DATS_MEMALLOC_LIBC -O2

TARGET=holospace_dag

all: $(TARGET)

$(TARGET): holospace_dag.sats holospace_dag.dats
	$(PATSCC) $(ATSFLAGS) -o $(TARGET) holospace_dag.sats holospace_dag.dats

typecheck: holospace_dag.sats holospace_dag.dats
	$(PATSCC) $(ATSFLAGS) -tcats holospace_dag.sats holospace_dag.dats

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET) holospace_dag_sats.c holospace_dag_dats.c

.PHONY: all typecheck run clean
