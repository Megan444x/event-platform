import React, { useState, useEffect } from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import 'react-big-calendar/lib/css/react-big-calendar.css';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL;

const localizer = momentLocalizer(moment);

interface Event {
  id: number;
  title: string;
  start: Date;
  end: Date;
  sessions?: Session[];
}

interface Session {
  id: number;
  eventId: number;
  speaker: string;
  location: string;
  startTime: Date;
  endTime: Date;
}

interface Attendee {
  id: number;
  name: string;
  email: string;
  sessionId: number;
}

const EventDashboard: React.FC = () => {
  const [events, setEvents] = useState<Event[]>([]);
  const [sessions, setSessions] = useState<Session[]>([]);
  const [attendees, setAttendees] = useState<Attendee[]>([]);

  useEffect(() => {
    fetch(`${API_BASE_URL}/events`)
      .then((res) => res.json())
      .then(setEvents);
  }, []);

  useEffect(() => {
    fetch(`${API_BASE_URL}/sessions`)
      .then((res) => res.json())
      .then(setSessions);
  }, []);

  useEffect(() => {
    fetch(`${API_BASE_URL}/attendees`)
      .then((res) => res.json())
      .then(setAttendees);
  }, []);

  const handleSelectEvent = (event: Event) => {
    console.log(`Selected event: ${event.title}`);
  };

  const eventStyleGetter = () => {
    return {
      style: {
        backgroundColor: 'lightblue',
      },
    };
  };

  return (
    <div>
      <Calendar
        localizer={localizer}
        events={events.map(event => ({...event, start: new Date(event.start), end: new Date(event.end)}))}
        startAccessor="start"
        endAccessor="end"
        style={{ height: 500 }}
        onSelectEvent={handleSelectEvent}
        eventPropGetter={eventStyleGetter}
      />
    </div>
  );
};

export default EventDashboard;